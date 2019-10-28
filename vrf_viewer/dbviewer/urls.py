from django.urls import path

from . import views

app_name = 'dbviewer'
urlpatterns = [
    path('', views.index, name='index'),
    path('<int:record_id>/', views.detail, name='detail'),
    path('refresh/', views.refresh, name='refresh'),
]