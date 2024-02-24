<!-- ### USERS ### -->
<!-- Add 2 Users -->
curl -X POST \
    http://127.0.0.1:16980/user \
    -H 'Content-Type: application/json' \
    -d '{"name": "Alice","active":true}'

curl -X POST \
    http://127.0.0.1:16980/user \
    -H 'Content-Type: application/json' \
    -d '{"name": "Bob","active":true}'

curl -X POST \
    http://127.0.0.1:16980/user \
    -H 'Content-Type: application/json' \
    -d '{"name": "Cat","active":true}'

curl -X POST \
    http://127.0.0.1:16980/user \
    -H 'Content-Type: application/json' \
    -d '{"name": "Daniel","active":true}'

<!-- Return Users -->
curl -X GET \
    http://127.0.0.1:16980/users

<!-- Return User by ID -->
curl -X GET \
    http://127.0.0.1:16980/user/id/1

<!-- Return User by name -->
curl -X GET \
    http://127.0.0.1:16980/user/name/Daniel

<!-- Return Users by activity -->
curl -X GET \
    http://127.0.0.1:16980/users/active/true

<!-- Update User -->
curl -X PATCH \
    http://127.0.0.1:16980/user/id/1 \
    -H 'Content-Type: application/json' \
    -d '{"id": 1, "name": "Daniel","active":false}'

<!-- Return User by activity (Confirm Change) -->
curl -X GET \
    http://127.0.0.1:16980/users/active/false

<!-- Delete User by ID -->
curl -X DELETE \
    http://127.0.0.1:16980/user/id/1

<!-- Return User by ID (Confirm Delete) -->
curl -X GET \
    http://127.0.0.1:16980/user/id/1

<!-- Delete User by name -->
curl -X DELETE \
    http://127.0.0.1:16980/user/name/Bob

<!-- Return User by name (Confirm Delete) -->
curl -X GET \
    http://127.0.0.1:16980/user/name/Bob

<!--  -->
<!-- ### CONVERSATIONS ### -->
<!--  -->

<!-- Add User -->

<!-- Add 2 Conversations -->
curl -X POST \
    http://127.0.0.1:16980/conversation/1 \
    -H 'Content-Type: application/json' \
    -d '{"id": null, "name: "fantastic", "user_id": 2}'

curl -X POST \
    http://127.0.0.1:16980/conversation/1 \
    -H 'Content-Type: application/json' \
    -d '{"id": null, "name: "testing", "user_id": 2}'

<!-- Return Conversations -->
curl -X GET \
    http://127.0.0.1:16980/conversations

<!-- Return Conversation by ID -->
curl -X GET \
    http://127.0.0.1:16980/conversation/id/1

<!-- Return Conversation by user_id_ -->
curl -X GET \
    http://127.0.0.1:16980/conversation/user/1

<!-- Update User -->
curl -X PATCH \
    http://127.0.0.1:16980/conversation/update/1 \
    -H 'Content-Type: application/json' \
    -d '{"id": null, "name: null, "user_id": 2}'

<!-- Delete Conversation by ID -->
curl -X DELETE \
    http://127.0.0.1:16980/conversation/id/1

<!-- Return Conversations -->
curl -X GET \
    http://127.0.0.1:16980/conversations

<!--  -->
<!-- ### ENGAGEMENTS ### -->
<!--  -->

<!-- Add User -->

<!-- Add engagement -->
curl -X POST \
    http://127.0.0.1:16980/engagement/new \
    -H 'Content-Type: application/json' \
    -d '{"conversation_id": 1, "query": "wtf?", "response": "I know, right?"}'

curl -X POST \
    http://127.0.0.1:16980/engagement/new \
    -H 'Content-Type: application/json' \
    -d '{"conversation_id": 1, "query": "seriously?", "response": "Fuck yeah!?"}'

<!-- Return engagements -->
curl -X GET \
    http://127.0.0.1:16980/engagements

<!-- Return engagement by ID -->
curl -X GET \
    http://127.0.0.1:16980/engagement/id/1

<!-- Return engagement by conversation_id -->
curl -X GET \
    http://127.0.0.1:16980/engagements/conversation/1

<!-- Update engagement -->
curl -X PATCH \
    http://127.0.0.1:16980/engagement/update/1 \
    -H 'Content-Type: application/json' \
    -d '{"id": null, "conversation_id": 2}'

<!-- Return engagement by conversation_id 2 (Check Result) -->
<!-- !TODO! -- Make not overwrite existing! -->
curl -X GET \
    http://127.0.0.1:16980/engagements/conversation/1

<!-- Delete engagement by ID -->
curl -X DELETE \
    http://127.0.0.1:16980/engagement/id/1

<!-- Return engagements -->
curl -X GET \
    http://127.0.0.1:16980/engagements