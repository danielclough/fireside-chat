curl http://127.0.0.1:3456/inference


<!-- Change  sample_len -->
curl -X PATCH \
    -H 'Content-Type: application/json' \
    -d '{"tracing":true,"temperature":0.2,"top_p":1.0,"seed":299792458,"sample_len":300,"repeat_penalty":1.3,"repeat_last_n":150}' \
    http://127.0.0.1:3456/inference
