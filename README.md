# SupNum-scan

**SupNum-scan** est un outil de reconnaissance réseau écrit en **Rust**, combinant un scan rapide natif et un scan avancé via **Nmap**.

Développé dans un cadre académique et orienté **sécurité offensive / CTF / pentesting**.

---

## ✨ Fonctionnalités

- 🚀 Scan rapide des ports TCP (Rust async)
- 📊 Rapport automatique par cible
- 🧠 Intégration Nmap (`-sV -A`) après confirmation
- 🗂️ Historique des scans par IP
- ⚡ Exécution rapide et propre

---

## 📦 Installation

### 🔹 Dépendances

- Linux (Kali recommandé)
- `nmap` installé
- Rust (uniquement pour compilation)

```bash
sudo apt install nmap

🔹 Compilation

git clone https://github.com/LeminEly/supnum-scan.git
cd supnum-scan
cargo build --release

🔹 Installation système (global)

sudo cp target/release/supnum-scan /usr/local/bin/

🚀 Utilisation

supnum-scan <ip | domaine>

Exemple :

supnum-scan 10.80.155.235

📄 Rapports

Les rapports sont sauvegardés automatiquement dans :

scans/

Format :

<ip>_1.txt
<ip>_2.txt

Chaque rapport contient :

    Ports ouverts

    Temps de scan

    Résultat Nmap (si exécuté)

🧱 Architecture

src/
├── main.rs
├── banner.rs
├── args.rs
├── output.rs
└── scanner/
    ├── portscan.rs
    └── nmap.rs

⚠️ Disclaimer

Cet outil est destiné uniquement à des fins éducatives et légales.
L’auteur n’est pas responsable d’un usage abusif.
👤 Auteur

Lemin Ely
Institut Supérieur du Numérique
GitHub : https://github.com/LeminEly

