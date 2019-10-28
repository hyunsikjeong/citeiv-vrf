from django.shortcuts import get_object_or_404, render

# Create your views here.
from django.http import HttpResponse, Http404
from .models import Records
import requests

def index(request):
    latest_record_list = Records.objects.order_by('-id')[:5]
    context = {'latest_record_list': latest_record_list}
    return render(request, 'dbviewer/index.html', context)

def detail(request, record_id):
    record = get_object_or_404(Records, pk=record_id)
    return render(request, 'dbviewer/detail.html', {'record': record})

def refresh(request):
    try:
        r = requests.get('http://rb-tree.xyz/citeivapi/size')
        size_api = r.json()['index']
        size_db = Records.objects.count()

        for i in range(size_db, size_api):
            r = requests.get('http://rb-tree.xyz/citeivapi/get/{}'.format(i + 1))
            data = r.json()

            record = Records(seed=data['seed'], input=data['input'], output=data['output'], proof=data['proof'])
            record.save()
        return HttpResponse("Refresh successful: {} to {}".format(size_db, size_api))
    except Exception as e:
        raise Http404("Failed to handle: {}".format(e))
