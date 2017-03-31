#include <stdio.h>

struct Node
{
  struct Node* l;
  struct Node* r;
  int is_head;
  int data;
};

void collect_leaves(struct Node* head[], int node_count, int p[], int* idx)
{
  if (node_count == 0)
    return;

  struct Node *curr, *next[node_count*2];
  int next_count = 0;
  for(int i=0; i<node_count; i++)
    {
      curr = head[i];
      if (curr->l == NULL && curr->r == NULL)
        {
          p[*idx] = curr->data;
          (*idx)++;
        }
      if (curr->l != NULL)
        {
          next[next_count] = curr->l;
          next_count++;
        }
      if (curr->r != NULL)
        {
          next[next_count] = curr->r;
          next_count++;
        }
    }
  collect_leaves(next, next_count, p, idx);
}

int main()
{
  int size;
  struct Node nodes[10], *head=NULL;
  char l=0, r=0;
  scanf("%d", &size);
  if (size == 0)
    return 0;

  for(int i=0; i<size; i++)
    {
      nodes[i].is_head = -1;
      nodes[i].data = i;
    }
  for(int i=0; i<size; i++)
    {
      scanf(" %c %c", &l, &r);
      if (l != '-')
        {
          nodes[i].l = nodes+(l - '0');
          (nodes+(l - '0'))->is_head = 0;
          if(nodes[i].is_head == -1)
            {
              nodes[i].is_head = 1;
            }
        }
      else
        {
          nodes[i].l = NULL;
        }
      if (r != '-')
        {
          nodes[i].r = nodes+(r - '0');
          (nodes+(r - '0'))->is_head = 0;
          if(nodes[i].is_head == -1)
            {
              nodes[i].is_head = 1;
            }
        }
      else
        {
          nodes[i].r = NULL;
        }
    }
  for(int i=0; i<size; i++)
    {
      if(nodes[i].is_head == 1)
        {
          head = &nodes[i];
          break;
        }
    }
  if(head == NULL)
    head = nodes;

  int p[10], idx=0;
  struct Node* heada[] = {head};
  collect_leaves(heada, 1, p, &idx);
  for (int i=0; i<idx-1; i++)
    {
      printf("%d ", p[i]);
    }
  if (idx > 0)
    {
      printf("%d", p[idx-1]);
    }
}
