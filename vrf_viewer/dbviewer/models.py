from django.db import models

# Create your models here.

class Records(models.Model):
    seed = models.CharField(max_length=64)
    input = models.CharField(max_length=200)
    output = models.CharField(max_length=64)
    proof = models.CharField(max_length=162)