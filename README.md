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
