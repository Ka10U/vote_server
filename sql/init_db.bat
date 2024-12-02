@echo off
REM Chemin vers le fichier SQL
set SQL_FILE=init_db.sql

REM Chemin vers la base de données SQLite
set DB_FILE=../votes.db

REM Exécuter le script SQL pour initialiser la base de données
sqlite3 %DB_FILE% < %SQL_FILE%

echo Database initialized successfully.
pause