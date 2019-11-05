from django.shortcuts import get_object_or_404, render

# Create your views here.
from django.http import HttpResponse, Http404
from .models import Records
import requests

def index(request):
    latest_record_list = Records.objects.order_by('-id')

    if latest_record_list is None:
        return render(request, 'dbviewer/index.html', {'table_header': None})

    table_header = Records.user_input_fields()
    table_header.append("Output")

    table_rows = []
    for record in latest_record_list:
        row = record.parse_user_input()
        row.append(record.get_user_output())
        table_rows.append({ 'id': record.id, 'idx': record.idx, 'values': row })

    context = {'table_header': table_header, 'table_rows': table_rows}
    return render(request, 'dbviewer/index.html', context)

def detail(request, record_id):
    record = get_object_or_404(Records, pk=record_id)
    user_input_table = record.get_user_input_table()
    user_output_table = record.get_user_output_table()
    context = {'record': record, 'uinput': user_input_table, 'uoutput': user_output_table}

    return render(request, 'dbviewer/detail.html', context)

def refresh(request):
    try:
        r = requests.get('http://rb-tree.xyz/citeivapi/size')
        size_api = r.json()['index']
        size_db = Records.objects.count()

        for i in range(size_db, size_api):
            r = requests.get('http://rb-tree.xyz/citeivapi/get/{}'.format(i + 1))
            data = r.json()

            record = Records(idx=i+1, seed=data['seed'], input=data['input'], output=data['output'], proof=data['proof'])
            record.save()
        return HttpResponse("Refresh successful: {} to {}".format(size_db, size_api))
    except Exception as e:
        raise Http404("Failed to handle: {}".format(e))
