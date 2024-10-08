from typing import Union
from fastapi import FastAPI
from pydantic import BaseModel
import json, specdoctor

app = FastAPI()


# post빋을 키:값을 미리 정의 해야함
class Item(BaseModel):
    OS: str
    CPU_len: int
    mem_used: float
    mem_total: float
    vmem_total: float


# ============================
@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.post("/myinfo/")
async def create_item(item: Item):
    info = json.loads(item.json())
    specdoctor.pas(info)
    return 200


@app.post("/info")
def info():
    return 200


@app.get("/items/{item_id}")
def read_item(item_id: int, q: Union[str, None] = None):
    return {"item_id": item_id, "q": q}
