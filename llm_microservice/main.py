from fastapi import FastAPI
from pydantic import BaseModel
from llama_cpp import Llama

app = FastAPI()

# Load the GGUF model
llm = Llama(
    model_path = "C:/Users/swaga/Documents/llm models/DeepSeek-R1-Distill-Llama-8B-Q4_K_M.gguf",
    n_threads = 6, 
    n_gpu_layers = 2,
    n_ctx = 2048,    # context window
    verbose=False # disable output logs
)

class GenerationRequest(BaseModel):
    input_text: str
    instruction: str
#f"You are a creative writing assistant.\n"
@app.post("/generate")
def generate_text(data: GenerationRequest):
    prompt = (
        f"Respond like a writing assistant.\n"
        f"Instruction: {data.instruction}\n"
        f"Text: {data.input_text}\n"
        f"Response:"
    )

    output = llm(prompt, max_tokens = 50, temperature = 0.7)
    return {"response": output["choices"][0]["text"].strip()}

    


