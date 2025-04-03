# Verwende ein Python-Image
FROM python:3.9-slim

# Setze das Arbeitsverzeichnis
WORKDIR /app

# Kopiere die Anforderungen und installiere sie

RUN pip install flask requests
# Kopiere den Flask-Code in das Arbeitsverzeichnis
COPY . ./


# Starte die Flask-Anwendung
CMD ["python", "app.py"]