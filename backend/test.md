<!-- INFERENCE -->

<!-- Get inference JSON -->
curl http://127.0.0.1:16981/inference

<!-- Change  sample_len -->
curl -X PATCH \
    http://127.0.0.1:16981/inference \
    -H 'Content-Type: application/json' \
    -d '{"temperature":0.2,"top_p":1.0,"seed":299792458,"sample_len":300,"repeat_penalty":1.3,"repeat_last_n":150}' \


<!-- CURRENT MODEL -->

<!-- Get current model JSON -->
curl http://127.0.0.1:16981/model

<!-- Change  quantized -->
curl -X PATCH \
    http://127.0.0.1:16981/model \
    -H 'Content-Type: application/json' \
    -d '{"repo_id":"DanielClough/Candle_Mistral-7B-Instruct-v0.1", "model_name":"Candle_Mistral-7B-Instruct-v0.1_q6k.gguf","revision":"main","tokenizer_file":null,"weight_files":null,"quantized":false,"use_flash_attn":false}'


<!-- MODEL LIST -->

<!-- Get model list JSON -->
curl http://127.0.0.1:16981/model-list


<!-- ROLE LIST -->

<!-- Get role list JSON -->
curl http://127.0.0.1:16981/role-list

<!-- RESTART -->

<!-- Get model-start -->
curl http://127.0.0.1:16981/model-start