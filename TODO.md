# TODO DU PROJET HUMANGL

* **I.** Remettre en forme le projet vulkanalia pour accueillir HumanGl
* **II.** Créer des modèles a partir d'une fonction et setup les membres.
* **III.** Préparer la Mutant Stack (Matrice Stack).
* **IV.** Ajouter une interface de modification des modèles.

## I. Remettre en forme vulkanalia -> tcros

* Empêcher l'affichage nécessaire d'un modèle
* Gérer l'affichage d'une fenêtre vide sans erreurs.
* Supprimer la gestion des textures
* Supprimer la gestion des modèles

## II. Créer des modèles à partir d'une focntion -> ?

* Créer une structure "Membres" -> ObjModel, points d'accroche
* fonction qui crée un modèle à partir de sa taille (x, y, z)
* fonction qui accroche ce modèle à un autre objet/position de départ.

## **III.** Préparer la Mutant Stack (Matrice Stack) -> nbacconn

* Créer une matrice stack afin de déterminer les positions de points d'accroche entre les membres
* fonction permettant de déplacer un membre "enfant". Devra modifier aussi les membres "parents"

## **IV.** Ajouter une interface de modification des modèles

* Rendre les variables codé en dur en tant que mut, utilisé par l'interface.
* afficher un menu
* rendre selectionnable les objets afin de les modifier
