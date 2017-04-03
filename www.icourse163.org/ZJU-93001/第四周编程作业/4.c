#include <stdio.h>
#include <stdlib.h>

typedef int ElementType;
typedef struct TNode *Position;
typedef Position BinTree;
struct TNode{
    ElementType Data;
    BinTree Left;
    BinTree Right;
};

BinTree Insert( BinTree BST, ElementType X );
BinTree Delete( BinTree BST, ElementType X );
Position Find( BinTree BST, ElementType X );
Position FindMin( BinTree BST );
Position FindMax( BinTree BST );

int main()
{
    BinTree BST, MinP, MaxP, Tmp;
    ElementType X;
    int N, i;

    BST = NULL;
    scanf("%d", &N);
    for ( i=0; i<N; i++ ) {
        scanf("%d", &X);
        BST = Insert(BST, X);
    }
    MinP = FindMin(BST);
    MaxP = FindMax(BST);
    scanf("%d", &N);
    for( i=0; i<N; i++ ) {
        scanf("%d", &X);
        Tmp = Find(BST, X);
        if (Tmp == NULL) printf("%d is not found\n", X);
        else {
            printf("%d is found\n", Tmp->Data);
            if (Tmp==MinP) printf("%d is the smallest key\n", Tmp->Data);
            if (Tmp==MaxP) printf("%d is the largest key\n", Tmp->Data);
        }
    }
    scanf("%d", &N);
    for( i=0; i<N; i++ ) {
        scanf("%d", &X);
        BST = Delete(BST, X);
    }

    return 0;
}

BinTree Insert(BinTree BST, ElementType X)
{
  if (BST == NULL)
    {
      BinTree new_node = malloc(sizeof(struct TNode));
      new_node->Data = X;
      new_node->Left = NULL;
      new_node->Right = NULL;
      return new_node;
    }
  Position insert_point = BST;
  while (1)
    {
      if (X < insert_point->Data)
        {
          if (insert_point->Left == NULL)
            {
              insert_point->Left = Insert(NULL, X);
              break;
            }
          else
            {
              insert_point = insert_point->Left;
            }
        }
      else
        {
          if (insert_point->Right == NULL)
            {
              insert_point->Right = Insert(NULL, X);
              break;
            }
          else
            {
              insert_point = insert_point->Right;
            }
        }
    }
  return BST;
}

BinTree Delete(BinTree BST, ElementType X)
{
  if (BST == NULL)
      goto notfound;

  if (X < BST->Data)
    {
      BST->Left = Delete(BST->Left, X);
      return BST;
    }
  else if (X > BST->Data)
    {
      BST->Right = Delete(BST->Right, X);
      return BST;
    }
  else
    {
      if (BST->Left && BST->Right)
        {
          BinTree min = FindMin(BST->Right);
          BST->Data = min->Data;
          BST->Right = Delete(BST->Right, min->Data);
          return BST;
        }
      else if (BST->Left)
        {
          BinTree new_bst = BST->Left;
          free(BST);
          return new_bst;

        }
      else if (BST->Right)
        {
          BinTree new_bst = BST->Right;
          free(BST);
          return new_bst;
        }
      else
        {
          free(BST);
          return NULL;
        }
    }
 notfound:
  printf("Not Found\n");
  return BST;
}

BinTree Find(BinTree BST, ElementType X)
{
  BinTree current_node = BST;
  while (current_node)
    {
      if (X < current_node->Data)
        {
          current_node = current_node->Left;
        }
      else if (X > current_node->Data)
        {
          current_node = current_node->Right;
        }
      else
        {
          return current_node;
        }
    }
  return current_node;
}

BinTree FindMin(BinTree BST)
{
  while (BST && BST->Left)
    {
      BST = BST->Left;
    }
  return BST;
}

BinTree FindMax(BinTree BST)
{
  while (BST && BST->Right)
    {
      BST = BST->Right;
    }
  return BST;
}
