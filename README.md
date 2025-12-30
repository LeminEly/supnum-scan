# SupNum-scan

**SupNum-scan** est un outil de reconnaissance réseau écrit en **Rust**, combinant un **scan TCP rapide natif** et un **scan avancé via Nmap**, dans un seul workflow simple et efficace.

Ce projet a été développé dans un **cadre académique**, avec une orientation **sécurité offensive, CTF et pentesting**.

---

## 🧠 Contexte du projet

Ce projet est né lors de longues sessions de tests où je passais constamment d’un outil à un autre  
(**RustScan → Nmap → rapports manuels** 😵‍💫).

Entre la fatigue, les commandes répétitives et la perte de temps, l’idée était simple :

> **Un seul outil, un seul lancement, un résultat clair.**

C’est ainsi qu’est né **SupNum-scan**.

---

## ✨ Fonctionnalités

- 🚀 Scan rapide des ports TCP (implémentation native en Rust, async)
- 📊 Génération automatique de rapports par cible
- 🧠 Intégration de **Nmap** (`-sV -A`) après confirmation utilisateur
- 🗂️ Historique des scans (fichiers numérotés par IP / cible)
- ⚡ Exécution rapide, propre et lisible
- 📁 Création automatique du dossier `scans/` dans le dossier courant

---

## 📦 Installation

### 🔹 Prérequis

- Linux (Kali Linux recommandé)
- `nmap` installé sur le système
- Rust (uniquement pour la compilation)

```bash
sudo apt update
sudo apt install nmap

🔹 Compilation

git clone https://github.com/LeminEly/supnum-scan.git
cd supnum-scan
cargo build --release

🔹 Installation globale (recommandée)

Permet d’utiliser l’outil depuis n’importe quel dossier :

sudo cp target/release/supnum-scan /usr/local/bin/

🚀 Utilisation

supnum-scan <ip | domaine>

Exemple

supnum-scan 10.80.155.235

    Le scan rapide s’exécute automatiquement

    Les ports ouverts sont affichés

    Une confirmation est demandée avant le scan Nmap avancé

📄 Rapports

Les rapports sont sauvegardés automatiquement dans :

./scans/

Format des fichiers

<ip>_1.txt
<ip>_2.txt

Contenu d’un rapport

    Liste des ports ouverts

    Temps total du scan

    Résultats complets de Nmap (si exécuté)

🧱 Architecture du projet

src/
├── main.rs
├── banner.rs
├── args.rs
├── output.rs
└── scanner/
    ├── portscan.rs
    └── nmap.rs

⚠️ Disclaimer (IMPORTANT)

🚨 AVERTISSEMENT LÉGAL 🚨

Cet outil est destiné exclusivement à :

    l’apprentissage

    la recherche académique

    les environnements de test autorisés

    les CTF

    les audits de sécurité avec autorisation explicite

❌ Il est strictement interdit d’utiliser cet outil pour scanner :

    des réseaux

    des systèmes

    des serveurs

qui ne vous appartiennent pas ou pour lesquels vous n’avez pas une autorisation écrite explicite.

👉 L’auteur ne pourra en aucun cas être tenu responsable d’un usage illégal, abusif ou non autorisé de cet outil.
👤 Auteur

Lemin Ely
Institut Supérieur du Numérique
GitHub : https://github.com/LeminEly
