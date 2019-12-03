from django.contrib import admin

# Register your models here.

from .models import VRFRecord, OutputInfo

admin.site.register(VRFRecord)
admin.site.register(OutputInfo)
