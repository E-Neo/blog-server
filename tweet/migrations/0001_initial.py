# -*- coding: utf-8 -*-
# Generated by Django 1.11.2 on 2017-06-18 07:21
from __future__ import unicode_literals

from django.db import migrations, models
import uuid


class Migration(migrations.Migration):

    initial = True

    dependencies = [
    ]

    operations = [
        migrations.CreateModel(
            name='Tweet',
            fields=[
                ('id', models.UUIDField(default=uuid.uuid4, editable=False, primary_key=True, serialize=False)),
                ('publish_time', models.DateTimeField(auto_now_add=True)),
                ('content', models.CharField(max_length=512)),
            ],
            options={
                'ordering': ['-publish_time'],
            },
        ),
    ]
