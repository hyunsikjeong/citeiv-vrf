use std::path::Path;

use rand::Rng;
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;

pub struct Database {
    conn: sqlite::Connection,
    vrf: ECVRF,
    secret_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub seed: String,
    pub input: String,
    pub output: String,
    pub proof: String,
}

const SEED_LEN: usize = 32;

impl Database {
    pub fn new<T: AsRef<Path>>(path: T, ciphersuite: CipherSuite, secret_key: Vec<u8>) -> Self {
        // TODO: error handling
        let conn = sqlite::open(path).unwrap();
        let vrf = ECVRF::from_suite(ciphersuite).unwrap();
        let result = conn.execute(
            "
            CREATE TABLE outputs (seed TEXT, user_input TEXT, output TEXT, proof TEXT);
            ",
        );
        match result {
            Ok(_) => println!("Created the table"),
            Err(_) => println!("Already the table exists"),
        }
        Self {
            conn,
            vrf,
            secret_key,
        }
    }

    fn insert_inner(&self, row: Row) {
        // TODO: remove '' from input
        // TODO: check hexstring-ness and length of seed/output/proof
        // TODO: error handling

        let mut statement = self
            .conn
            .prepare("INSERT INTO outputs VALUES(?, ?, ?, ?)")
            .unwrap();

        statement.bind(1, &*row.seed).unwrap();
        statement.bind(2, &*row.input).unwrap();
        statement.bind(3, &*row.output).unwrap();
        statement.bind(4, &*row.proof).unwrap();

        statement.next().unwrap();
    }

    pub fn insert(&mut self, user_input: String) {
        let seed = self.get_seed();

        let mut input = vec![];
        input.extend(&seed);
        input.extend(user_input.as_bytes());

        let pi = self.vrf.prove(&self.secret_key, &input).unwrap();
        let hash = self.vrf.proof_to_hash(&pi).unwrap();

        let pi_str = hex::encode(pi);
        let hash_str = hex::encode(hash);

        self.insert_inner(Row {
            seed: hex::encode(seed),
            input: user_input,
            output: hash_str,
            proof: pi_str,
        });
    }

    pub fn get_row(&self, num: i64) -> Row {
        // Statement setting
        let mut statement = self
            .conn
            .prepare("SELECT * FROM outputs WHERE rowid = ?")
            .unwrap();
        statement.bind(1, num).unwrap();

        // Read
        statement.next().unwrap();
        Row {
            seed: statement.read::<String>(0).unwrap(),
            input: statement.read::<String>(1).unwrap(),
            output: statement.read::<String>(2).unwrap(),
            proof: statement.read::<String>(3).unwrap(),
        }
        // TODO: There must be one row
    }

    pub fn size(&self) -> i64 {
        let mut statement = self.conn.prepare("SELECT COUNT(*) FROM outputs").unwrap();

        statement.next().unwrap();
        statement.read::<i64>(0).unwrap()
    }

    fn get_seed(&self) -> Vec<u8> {
        let size = self.size();
        match size {
            0 => {
                let mut rng = rand::thread_rng();
                (0..SEED_LEN).map(|_| rng.gen::<u8>()).collect()
            }
            v => hex::decode(self.get_row(v).output).unwrap(),
        }
    }

    pub fn pubkey(&mut self) -> Vec<u8> {
        self.vrf.derive_public_key(&self.secret_key).unwrap()
    }
}
