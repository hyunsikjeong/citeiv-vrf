from django.db import models

# Create your models here.

class Records(models.Model):
    idx = models.BigIntegerField()
    seed = models.CharField(max_length=64)
    input = models.CharField(max_length=200)
    output = models.CharField(max_length=64)
    proof = models.CharField(max_length=162)

    @staticmethod
    def user_input_fields():
        return ["NAME", "TYPE", "WOW"]
    
    def parse_user_input(self):
        ln = len(self.user_input_fields())
        res = self.input.split('|', ln)
        return res

    def get_user_output(self):
        out = int(self.output, 16) % 3
        res = ['Gold', 'Silver', 'Bronze']
        return "{}({})".format(out, res[out])

    def get_user_input_table(self):
        res = []
        input_fields = self.user_input_fields()
        user_input = self.parse_user_input()

        for i in range(len(input_fields)):
            col = {'type': input_fields[i]}
            if i < len(user_input):
                col['value'] = user_input[i]
            else:
                col['value'] = 'None'
            res.append(col)
        
        if len(user_input) > len(input_fields):
            assert len(user_input) == len(input_fields) + 1
            res.append({'type': 'Extra Input', 'value': user_input[-1]})

        return res

    def get_user_output_table(self):        
        return [{'type': 'Output', 'value': self.get_user_output()}]
