# Generated by Django 2.2.6 on 2019-11-12 04:23

from django.db import migrations


class Migration(migrations.Migration):

    dependencies = [
        ('dbviewer', '0002_records_idx'),
    ]

    operations = [
        migrations.RenameModel(
            old_name='Records',
            new_name='VRFRecord',
        ),
    ]
