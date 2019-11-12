from django.shortcuts import get_object_or_404, render

# Create your views here.
from django.http import HttpResponse, Http404
from .models import VRFRecord, OutputInfo


def static(**kwargs):
    def decorate(func):
        for k in kwargs:
            setattr(func, k, kwargs[k])
        return func

    return decorate


@static(last_update=None, loading=False)
def index(request):
    import time, threading
    cur_time = time.time()
    last_update = index.last_update
    if not index.loading and (last_update is None
                              or cur_time - last_update > 10):
        print("Last update: {}".format(last_update))
        print("Current time: {}".format(cur_time))

        index.loading = True
        t = threading.Thread(target=refresh, args=())
        t.start()
        index.last_update = cur_time

    latest_record_list = VRFRecord.objects.order_by('-id')

    if latest_record_list is None:
        return render(request, 'dbviewer/index.html', {'table_header': None})

    table_header = VRFRecord.user_input_fields()
    table_header.append("Output Value")
    table_header.append("Output Name")

    table_rows = []
    for record in latest_record_list:
        row = record.parse_user_input()
        user_output = record.get_user_output()

        row.append(user_output)
        
        try:
            output_info = OutputInfo.objects.get(pk=user_output)
            row.append(output_info.name)
        except OutputInfo.DoesNotExist:
            row.append("None")
        
        table_rows.append({'id': record.id, 'idx': record.idx, 'values': row})

    context = {'table_header': table_header, 'table_rows': table_rows}
    return render(request, 'dbviewer/index.html', context)


def detail(request, record_id):
    record = get_object_or_404(VRFRecord, pk=record_id)
    user_input_table = record.get_user_input_table()
    user_output_table = record.get_user_output_table()

    user_output = record.get_user_output()

    output_info = get_object_or_404(OutputInfo, pk=user_output)
    user_output_table.extend([{
        'type': 'Output Name',
        'value': output_info.name
    }, {
        'type': 'Output Image',
        'image': output_info.image,
    }])


    context = {
        'record': record,
        'uinput': user_input_table,
        'uoutput': user_output_table
    }

    return render(request, 'dbviewer/detail.html', context)


def refresh():
    import requests
    try:
        r = requests.get('http://rb-tree.xyz/citeivapi/size')
        size_api = r.json()['index']
        size_db = VRFRecord.objects.count()

        for i in range(size_db, size_api):
            r = requests.get(
                'http://rb-tree.xyz/citeivapi/get/{}'.format(i + 1))
            data = r.json()

            record = VRFRecord(
                idx=i + 1,
                seed=data['seed'],
                input=data['input'],
                output=data['output'],
                proof=data['proof'])
            record.save()
        print("Refresh successful: {} to {}".format(size_db, size_api))
    except Exception as e:
        print("Failed to handle: {}".format(e))
    index.loading = False
