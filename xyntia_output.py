import sys
import re
import os

lines = sys.stdin.read().splitlines()

filename = None
smtlib_expression = None

for line in lines:
    if "instance_" in line and filename is None:
        filename = os.path.basename(line.strip())
        
    if line.startswith("smtlib:"):
        match = re.search(r'smtlib:\s*(.*)', line)
        if match:
            smtlib_expression = match.group(1).strip()
    
if filename and smtlib_expression:
    print(f"{smtlib_expression} {filename}")
else:
    print("Erreur : Impossible de trouver l'expression SMT-LIB ou le nom de fichier dans l'entrée.")