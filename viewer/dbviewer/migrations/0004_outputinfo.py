# Generated by Django 2.2.6 on 2019-11-12 04:27

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('dbviewer', '0003_auto_20191112_1323'),
    ]

    operations = [
        migrations.CreateModel(
            name='OutputInfo',
            fields=[
                ('id', models.AutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('image', models.ImageField(upload_to='')),
                ('name', models.CharField(max_length=200)),
            ],
        ),
    ]