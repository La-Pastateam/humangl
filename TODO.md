# TODO DU PROJET HUMANGL

* **I.** Remettre en forme le projet vulkanalia pour accueillir HumanGl
* **II.** Créer des modèles a partir d'une fonction et setup les membres.
* **III.** Préparer la Mutant Stack (Matrice Stack).
* **IV.** Ajouter une interface de modification des modèles.
* **V.** Finitions

## I. Remettre en forme vulkanalia -> tcros

* Ajouter une sortie propre avec echap ✅
* ~~Gérer l'affichage d'une fenêtre vide sans erreurs.~~
* Supprimer la gestion des textures ✅

## II. Créer des modèles à partir d'une focntion -> tcros

* Créer une structure "Membre" -> ObjModel, points d'accroche
* fonction qui crée un modèle à partir de sa taille (l, L, h)
* Supprimer la gestion des modèles par fichier local
* fonction qui accroche ce modèle à un autre objet/position de départ.

## **III.** Préparer la Mutant Stack (Matrice Stack) -> nbacconn

* Créer une matrice stack afin de déterminer les positions de points d'accroche entre les membres
* fonction permettant de déplacer un membre "enfant". Devra modifier aussi les membres "parents"
* fonction qui fait une translation entre 2 points

## **IV.** Ajouter une interface de modification des modèles

* Rendre les variables codé en dur en tant que mut, utilisé par l'interface.
* afficher un menu
* rendre selectionnable les objets afin de les modifier

## **V.** Finitions
* Remettre le code bien dans des dossiers (vulkan par exemple)

## **INFO**
* watch -n1 "grep VmRSS /proc/$(pidof humangl)/status"  pour tester la mise en VRAM du projet, pour les fuites mémoires.
