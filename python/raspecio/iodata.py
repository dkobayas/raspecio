from pydantic import BaseModel # type: ignore



class Input(BaseModel):
    a: int = 1
    b: int = 2

class Output(BaseModel):
    result: str