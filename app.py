from flask import Flask, render_template, send_from_directory

app = Flask(__name__)

@app.route('/')
def home():
    return render_template('base.html')

@app.route('/pkg/<path:filename>')
def serve_pkg(filename):
    return send_from_directory('pkg', filename)

@app.route('/fermat')
def serve_fermat():
    return render_template("fermat.html")
# @app.route('/static/<path:filename>')
# def serve_static(filename):
#     return send_from_directory('static', filename)

if __name__ == '__main__':
    app.run()