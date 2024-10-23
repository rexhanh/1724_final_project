# Motivation

Our team has two members, and we are both interested in large language models, which inspired us to build a tool that can help us serve LLMs as an API call and a command line tool with which people can interact.
We will build a tool similar to Ollama called rustllama, which helps run LLM locally. Ollama is written in Go, so our tool written in Rust will be a direct comparison to it.

1. We choose to do this in Rust because it will bring better performance to our local computer for inference, which might not have the best hardware.
2. We also need asynchronous execution since we want to build an API to serve the LLM concurrently. Rust also provides high performance in asynchronous executions with frameworks such as Rocket.

# Objective and key features

We will use `Rocket` to build APIs, and `Mistral.rs` for inference, we might use `sqlite` to handle database related tasks.

## Objective 1: `rustllama serve`

This command will start serving APIs at http://locahost:11435

### APIs

1. Create a model

   **Request**

   `curl http://localhost:11435/api/create -d '{"name":"modelname", "modelpath":"path/to/model.gguf"}'`

   **Response**

   ```
   {"status":"success"}
   {"status":"failed to create model"}
   ```

2. List local models

   **Request**

   `curl http://localhost:11435/api/list`

   **Response**

   ```
   {
    "models": [
        {
            "name":"llama3.2",
            "modified_at": "2024-10-04T14:56:49.277302595-07:00",
            "format":"gguf",
            "path":"path/to/model.gguf"
        }
        ...
    ]
   }
   ```

3. Delete a model

   **Request**

   `curl -X DELETE http://localhost:11435/api/delete -d '{"name":"llama3.2"}'`

   **Response**

   Returns 200 OK if successful, else 404 if the model does not exist.

4. Chat

   Generate the next chat message with a model managed by rustllama.

- `model`: (required) the model name
- `messages`: the messages of the chat will be sent to the llm, can be used to keep chat history
  The message object has those fields:
- `role`: the role of the message, either `system`, `assistant`, `user`
- `content`: the content of the message

  **Request**

  ```
  curl http://localhost:11435/api/chat -d '{
  "model":"llama3.2",
  "messages": [
    {
        "role":"user",
        "content":"why is the sky blue?"
    }
  ]
  }'
  ```

  **Response**

  ```
  {
    "model":"llama3.2",
    "message":{
        "role":"assistant",
        "content":"The sky is blue because..."
    }
  }
  ```

## Objective 2: `rustllama run <modelname>`

This command will use the `http://localhost:11435/api/chat` above, but it will enable us to chat on the command line without manually making a request.

# Tentative plan
