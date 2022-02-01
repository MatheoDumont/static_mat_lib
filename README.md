# Static Matrix Library

L'idée est d'implémenter une librairie permettant de créer des matrices de tailles statique uniquement :
  
```rust

use matrix::Matrix;

type Mat3 = Matrix::<f32, 3, 3>;

fn main() {
  let m = Mat3::zero();
  ...
}
```

Et de fournir les opérations de base. 
 

## Utilités:
Dans le cas où le type est connu à l'avance, qu'il ne changera pas et que la taille des matrices est aussi connue d'avance ce qui peut être souvent le cas.
Donc dans le cas où l'on n'a pas besoin d'avoir des du genre `matrix<type, n, n>` avec n inconnu.
 
## Avantages:
- Pour l'implementation, le compilateur aide beaucoup pour détecter les erreurs de tailles
- Pas d'allocation dynamique vu que la taille est connue à la compilation donc plus efficient
 
    
## Désavantages:
- Implémentation difficile, besoin d'aborder différement les choses.
- Le binary peut être bien plus lourd, car pour chaque type déclaré, il écrira toutes les fonctions avec ROW, COL spécialisés.
- Evidemment ,aucunes actions où la taille est inconnue à la compilation ne peuvent être accomplies.
- Comme la taille est déterminée à la compilation, on ne peut pas effectuer certaines actions comme transposer la matrice *in place*, et juste la modifier, on doit créer une nouvelle matrice.

- Rust ne fournit pas comme il appelle de *impl blanket*, de specialization des impl d'opérateurs qui sont encore unstable (RFC date de 2016!)
donc je ne peut pas surcharger l'opérateur pour spécialiser pour des choses utiles mais pas nécessaires.

- Comme je ne peut pas surcharger les opérateurs, l'implémentation de fonctionnalité comme le dot product devient compliqué: il s'agit juste sinon de la multiplication d'un vecteur ligne et d'un vecteur colonne, mais si les 2 sont inversés alors il y aura erreur, ce demanderait d'utiliser un if pour le savoir
ce qui est un calcul non nécessaire si je sais qu'il s'agit de toute façon de vecteur et qu'avec un objet vecteur, je pourrais juste faire `vec1.dot(vec2)`.
Une spécialization permettrait de définir la surcharger de l'opérateur * pour qu'il agisse différement si les matrix sont de la forme `matrix<Type, 3,1>` ou `matrix<Type, 1, 3>` quand on essaie de les multiplier entre elle.



## Conclusion
J'ai laissé tombé l'implémentation dans mon projet précédent et l'ai mis en pour y revenir un jour, si ca me prend. J'ai supprimé beaucoup de ligne pour les impl infaisables comme les 'impl blanket' ou la spesialization.




 

