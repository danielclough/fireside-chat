<!-- Get JSON -->
curl http://127.0.0.1:3456/inference


<!-- Change  sample_len -->
curl -X PATCH \
    http://127.0.0.1:3456/inference \
    -H 'Content-Type: application/json' \
    -d '{"temperature":0.2,"top_p":1.0,"seed":299792458,"sample_len":300,"repeat_penalty":1.3,"repeat_last_n":150}' \


<!-- Get JSON -->
curl http://127.0.0.1:3456/model


<!-- Change  cpu -->
curl -X PATCH \
    http://127.0.0.1:3456/model \
    -H 'Content-Type: application/json' \
    -d '{"model_id":"lmz/candle-mistral","revision":"main","tokenizer_file":null,"weight_files":null,"quantized":false,"use_flash_attn":false,"cpu":false}'
