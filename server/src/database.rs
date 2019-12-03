use std::fmt::{Display, Formatter, Result as FormatResult};
use std::path::Path;

use rand::Rng;
use vrf::openssl::{CipherSuite, Error as VRFError, ECVRF};
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

#[derive(Debug)]
pub enum Error {
    SqliteError(sqlite::Error),
    VRFError(VRFError),
    FromHexError(hex::FromHexError),
    WrongRowError(String),
}

impl From<sqlite::Error> for Error {
    fn from(error: sqlite::Error) -> Self {
        Error::SqliteError(error)
    }
}

impl From<VRFError> for Error {
    fn from(error: VRFError) -> Self {
        Error::VRFError(error)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(error: hex::FromHexError) -> Self {
        Error::FromHexError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        match self {
            Error::SqliteError(e) => write!(f, "Sqlite error: {}", e),
            Error::VRFError(e) => write!(f, "ECVRF error: {}", e),
            Error::FromHexError(e) => write!(f, "Hex error: {}", e),
            Error::WrongRowError(e) => write!(f, "Wrong row error: {}", e),
        }
    }
}

impl Database {
    pub fn new<T: AsRef<Path>>(
        path: T,
        ciphersuite: CipherSuite,
        secret_key: Vec<u8>,
    ) -> Result<Self, Error> {
        let conn = sqlite::open(path)?;
        let vrf = ECVRF::from_suite(ciphersuite)?;

        // If there's no table, create a table
        let result = conn.execute(
            "
            CREATE TABLE outputs (seed TEXT, user_input TEXT, output TEXT, proof TEXT);
            ",
        );
        match result {
            Ok(_) => println!("Created the table"),
            Err(_) => println!("Already the table exists"),
        }

        // Return
        Ok(Self {
            conn,
            vrf,
            secret_key,
        })
    }

    fn insert_inner(&self, row: Row) -> Result<(), Error> {
        if row.seed.len() != 64 {
            return Err(Error::WrongRowError(format!(
                "Expected length of seed is 64, but {}",
                row.seed.len()
            )));
        }
        if row.output.len() != 64 {
            return Err(Error::WrongRowError(format!(
                "Expected length of output is 64, but {}",
                row.output.len()
            )));
        }
        if row.proof.len() != 162 {
            return Err(Error::WrongRowError(format!(
                "Expected length of output is 162, but {}",
                row.proof.len()
            )));
        }

        let mut statement = self
            .conn
            .prepare("INSERT INTO outputs VALUES(?, ?, ?, ?)")?;

        statement.bind(1, &*row.seed)?;
        statement.bind(2, &*row.input)?;
        statement.bind(3, &*row.output)?;
        statement.bind(4, &*row.proof)?;

        statement.next()?;
        Ok(())
    }

    pub fn insert(&mut self, user_input: String) -> Result<(), Error> {
        let seed = self.get_seed()?;

        let mut input = vec![];
        input.extend(&seed);
        input.extend(user_input.as_bytes());

        let pi = self.vrf.prove(&self.secret_key, &input)?;
        let hash = self.vrf.proof_to_hash(&pi)?;

        let pi_str = hex::encode(pi);
        let hash_str = hex::encode(hash);

        self.insert_inner(Row {
            seed: hex::encode(seed),
            input: user_input,
            output: hash_str,
            proof: pi_str,
        })
    }

    pub fn get_row(&self, num: i64) -> Result<Row, Error> {
        // Statement setting
        let mut statement = self.conn.prepare("SELECT * FROM outputs WHERE rowid = ?")?;
        statement.bind(1, num)?;

        // Read
        statement.next()?;
        Ok(Row {
            seed: statement.read::<String>(0)?,
            input: statement.read::<String>(1)?,
            output: statement.read::<String>(2)?,
            proof: statement.read::<String>(3)?,
        })
    }

    pub fn size(&self) -> Result<i64, Error> {
        let mut statement = self.conn.prepare("SELECT COUNT(*) FROM outputs")?;

        statement.next()?;
        Ok(statement.read::<i64>(0)?)
    }

    fn get_seed(&self) -> Result<Vec<u8>, Error> {
        let size = self.size()?;
        match size {
            0 => {
                let mut rng = rand::thread_rng();
                Ok((0..SEED_LEN).map(|_| rng.gen::<u8>()).collect())
            }
            v => Ok(hex::decode(self.get_row(v)?.output)?),
        }
    }

    pub fn pubkey(&mut self) -> Result<Vec<u8>, Error> {
        Ok(self.vrf.derive_public_key(&self.secret_key)?)
    }
}
