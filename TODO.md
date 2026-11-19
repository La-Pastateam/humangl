# TODO DU PROJET HUMANGL

* **I.** Remettre en forme le projet vulkanalia pour accueillir HumanGl
* **II.** Créer des modèles a partir d'une fonction et setup les membres.
 A. Bones
 B. Skeleton
 C. Verifications
* **III.** Rebdre Graphique le Skeelton
* **IV.** Préparer la Mutant Stack (Matrice Stack).
* **V.** Ajouter une interface de modification des modèles.
* **VI.** Finitions

## I. Remettre en forme vulkanalia -> tcros

* Ajouter une sortie propre avec echap ✅
* ~~Gérer l'affichage d'une fenêtre vide sans erreurs.~~
* Supprimer la gestion des textures ✅

## II. Créer des modèles à partir d'une focntion -> tcros

### A. Bones

* Créer une structure ~"Membre"~ "Bone" -> ObjModel, ~~points d'accroche~~ matrice transformation homogène ✅
* Modifier ObjModel pour l'optimiser pour les Bones ✅
  * Le sujet stipule de créer une box en 1x1x1 que l'on transforme ensuite, plus d'objModel par Bone.
* fonction qui crée un modèle à partir de sa taille (l, `L, h) ✅
* ~fonction qui accroche ce modèle à un autre objet/position de départ.~

> Le pivot et la transformation se fait maintenant au niveau du calcul avec la matrice stack.

### B. Skeleton

* Créer une structure "Skeleton" -> Vecteur de struct Bone.✅
* Ajouter la fonction Human -> Skeleton de 10 bones : Head, Torso, 2 arms (with 1 upper arm and forearm), 2 legs (thigh + lower part)✅

### C. Vérif all

* Ajouter les fonctions utilitaires de Bones et Skeleton✅
  * Ajout d'un builder pour les os ✅
* ~fonction qui permet de resize l'objModel ?~

> Non nécessaire, l'objet peut se resize grace à `scale()` appliqué à la ~matrice transformation de l'os~ au vecteur dimension de l'os

* Changement du Bone => Plus de matrice4 générale mais juste les dimension, la rotation et le point pivot sous forme de vec3.✅
* Determiner où placer le point Pivot

## **III.** Rendre Graphique le skeleton -> tcros

* Supprimer la gestion des modèles par fichier local✅
* Ajouter le skeleton comme objet à render : l'app contient 1 skeleton qui ensuite render tous ses bones✅

> Précision : on load un model de `BoxMesh`, on draw ensuite plusieurs copie de ce box pour chaque os.

* possibilité d'afficher plusieurs ~models~ Bone.✅
* Ajouter le ~model skeleton à la swapchain~ skeleton et ses os à l'App afin de draw chacun d'entres eux en passant par le `update_secondary_buffer`.✅
*

## **IV.** Préparer la Mutant Stack (Matrice Stack) -> nbacconn

* Créer une matrice stack afin de déterminer les positions de points d'accroche entre les membres
* fonction permettant de déplacer un membre "enfant". Devra modifier aussi les membres "parents"
* fonction qui fait une translation entre 2 points
  * Formula : T⁻¹RT, On fait une translation du bone a partir de l'origin, on applique la rotation puis on réapplique la translation.

## **V.** Ajouter une interface de modification des modèles -> nbacconn

* Set up le back de l'interface utilisateur ✅
  * creation de nouvelle pipeline, command buffer, etc...
  * creation de nouveaux shaders
  * Compilation auto des shaders dans le ```compile.sh```
* Set up l'ui à notre goût ✅
* Rendre les variables codé en dur en tant que mut, utilisé par l'interface.
* afficher un menu
* rendre selectionnable les objets afin de les modifier

## **VI.** Finitions

* Remettre le code bien dans des dossiers (vulkan par exemple)
* Adapter le main a winit 0.30 ✅

## **INFO**

* watch -n1 "grep VmRSS /proc/$(pidof humangl)/status"  pour tester la mise en VRAM du projet, pour les fuites mémoires.
