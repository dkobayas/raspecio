from fastapi import FastAPI
from raspecio import raspecio, iodata

app = FastAPI(
    root_path="/raspecio-api"
)


@app.get("/")
def read_root():
    return {"message": "Hello World"}

@app.post("/sum_as_string")
def sum_as_string(input: iodata.Input) -> iodata.Output:
    a = input.a
    b = input.b
    return {"result": raspecio.sum_as_string(a, b)}