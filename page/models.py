from django.db import models


class Page(models.Model):
    name = models.CharField(max_length=64)
    content = models.TextField()
    script = models.TextField(blank=True)

    def __str__(self):
        return self.name


class Vocabulary(models.Model):
    word = models.CharField(max_length=128)
    datetime = models.DateTimeField(auto_now_add=True)
    status = models.CharField(
        max_length=128,
        choices=(('fresh', 'fresh'),
                 ('review', 'review'))
    )
    info = models.TextField(blank=True)

    def __str__(self):
        return self.word

    class Meta:
        verbose_name_plural = "vocabularies"
        ordering = ['-datetime']
