from app import app
from flask_frozen import Freezer
import os

freezer = Freezer(app)

@freezer.register_generator
def serve_pkg():
    for file in os.listdir("pkg"):
        if file.split(".")[-1] in ["js","wasm"]:
            yield {'filename': file}

@freezer.register_generator
def serve_template():
    for file in os.listdir("templates"):
        if file != "base.html":
            yield {'filename': file}

if __name__ == '__main__':
    freezer.freeze()