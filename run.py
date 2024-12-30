from raspecio import raspecio
from raspecio import fastapi

import uvicorn # type: ignore


def main():
    uvicorn.run(fastapi.app, host="localhost", port=8000)

if __name__ == '__main__':
    main()
   