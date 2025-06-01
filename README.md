# distSystemsShopApp

## Installation
```bash
docker-compose build
docker-compose up
```

## Projektstruktur
- **frontendFlask**: Flask-Frontend mit einem HTML-Template zur Interaktion mit dem Shop.
- **shop**: Rust-Backend auf Basis von Rocket, angebunden an PostgreSQL.
- **docker-compose.yml**: Koordiniert den Start aller Services (Backend, Frontend, Datenbank).

## Verteilte Systeme
In modernen verteilten Systemen wird eine Applikation in mehrere Services aufgeteilt, die über Netzwerke kommunizieren. Diese Architektur erhöht die Skalierbarkeit, Wartbarkeit und Fehlertoleranz eines Systems.
Dieses Projekt ist ein einfaches Beispiel für ein verteiltes System:
- Frontend-Service (Flask)
- Backend-Service (Rust Rocket API)
- Datenbank-Service (PostgreSQL)
Diese Trennung erlaubt unabhängiges Deployen, Skalieren und Entwickeln einzelner Komponenten.

## 12-Factor App Methodology
1. Codebase: Eine einzige Codebasis pro Anwendung, versioniert mit Git.
2. Dependencies: Klare Deklaration der Abhängigkeiten über Cargo.toml und requirements.txt (bei Python).
3. Config: Konfiguration erfolgt über Umgebungsvariablen (über Docker).
4. Backing Services: Die Datenbank wird als externer Service behandelt.
5. Build, Release, Run: Trennung von Build- und Runtime über Docker.
6. Processes: Die Anwendung läuft in stateless Prozessen, die leicht horizontal skaliert werden können.
7. Port Binding: Services binden an Ports und werden via HTTP verfügbar gemacht.
8. Concurrency: Mehrere Prozesse oder Container könnten parallel betrieben werden.
9. Disposability: Schnelles Starten und Stoppen von Containern.
10. Dev/Prod Parity: Entwicklung, Testing und Produktion laufen in möglichst ähnlichen Umgebungen.
11. Logs: Standard-Output der Container wird als Log-Quelle genutzt.
12. Admin Processes: Administrative Tasks (wie DB-Migrationen) könnten als einmalige Jobs gestartet werden.

## Docker mit Traefik
Für produktionsnähere Umgebungen kann Traefik als Reverse Proxy verwendet werden, um Routing, HTTPS und Load Balancing zu ermöglichen.

Traefik übernimmt:
- Weiterleitung von HTTP-Anfragen an das passende Backend oder Frontend
- Automatische Erkennung von Diensten über Docker-Labels
- TLS-Zertifikate

Projektstart:
```bash
docker-compose -f docker-compose-traefik.yml up --build
```

In der docker-compose-traefik.yml:
- Jeder Service erhält ein Label für Traefik (z. B. traefik.http.routers.frontend.rule=Host(...))
- Traefik lauscht auf Port 80
- Die Frontend-App nutzt API_SERVER_URL=http://rustshop:8000 zum Ansprechen des Backends

## Kubernetes Deployment
Für skalierbare und cloud-native Deployments kann dieses Projekt auch auf Kubernetes laufen.

Ein Beispiel für das Frontend:
```bash
apiVersion: apps/v1
kind: Deployment
metadata:
  name: frontend
spec:
  replicas: 1
  selector:
    matchLabels:
      app: frontend
  template:
    metadata:
      labels:
        app: frontend
    spec:
      containers:
        - name: frontend
          image: dein-dockerhub-user/frontend:latest
          ports:
            - containerPort: 5050
          env:
            - name: PORT
              value: "5050"
            - name: API_SERVER_URL
              value: "http://rustshop:8000"
---
apiVersion: v1
kind: Service
metadata:
  name: frontend
spec:
  selector:
    app: frontend
  ports:
    - protocol: TCP
      port: 80
      targetPort: 5050
  type: ClusterIP
```

In einer Kubernetes-Umgebung werden:
- Services über Service-Ressourcen adressierbar gemacht
- Umgebungsvariablen (z. B. API_SERVER_URL) verwendet, um Kommunikation zwischen Pods zu ermöglichen
- Traefik alternativ als Ingress Controller eingesetzt