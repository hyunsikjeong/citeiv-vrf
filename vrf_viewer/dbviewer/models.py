from django.db import models

# Create your models here.
import inspect


class VRFRecord(models.Model):
    idx = models.BigIntegerField()
    seed = models.CharField(max_length=64)
    input = models.CharField(max_length=200)
    output = models.CharField(max_length=64)
    proof = models.CharField(max_length=162)

    @staticmethod
    def user_input_fields():
        return ["NAME", "TYPE"]

    def parse_user_input(self):
        ln = len(self.user_input_fields())
        res = self.input.split('|', ln)
        if len(res) < ln + 1:
            res += ["None"] * (ln + 1 - len(res))
        return res

    def get_user_output(self):
        out = int(self.output, 16) % 6 + 1
        return out

    def get_user_output_logic(self):
        source = inspect.getsource(self.get_user_output)
        output = ""
        for line in source.split('\n'):
            # Assuming one tab is 4 spaces
            output += line[4:] + '\n'
        return output[:-1]

    def get_user_input_table(self):
        res = []
        input_fields = self.user_input_fields()
        user_input = self.parse_user_input()

        for i in range(len(input_fields)):
            col = {'type': input_fields[i], 'value': user_input[i]}
            res.append(col)

        if len(user_input) > len(input_fields):
            assert len(user_input) == len(input_fields) + 1
            res.append({'type': 'Extra Input', 'value': user_input[-1]})

        return res

    def get_user_output_table(self):
        res = [{
            'type': 'Output',
            'value': self.get_user_output()
        }, {
            'type': 'Output Logic',
            'value': self.get_user_output_logic()
        }]
        return res

class OutputInfo(models.Model):
    image = models.ImageField()
    name = models.CharField(max_length=200)

