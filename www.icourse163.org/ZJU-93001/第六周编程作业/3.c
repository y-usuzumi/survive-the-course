#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>

struct _AdjacencyList
{
  int value;
  struct _AdjacencyList *next;
};
typedef struct _AdjacencyList AdjacencyList;

struct _SDSGraph
{
  AdjacencyList **adjacency_lists;
  int size;
};
typedef struct _SDSGraph SDSGraph;

struct _Queue
{
  AdjacencyList* head;
  AdjacencyList* tail;
};
typedef struct _Queue Queue;

Queue* init_queue()
{
  Queue* q = malloc(sizeof(Queue));
  q->head = NULL;
  q->tail = NULL;
  return q;
}

void enqueue(Queue* q, int num)
{
  AdjacencyList* l = malloc(sizeof(AdjacencyList));
  l->value = num;
  l->next = NULL;
  if (q->head == NULL)
    {
      q->head = l;
    }
  if (q->tail != NULL)
    {
      q->tail->next = l;
    }
  q->tail = l;
}

int dequeue(Queue* q)
{
  if (q->head == NULL)
    {
      printf("Dequeuing empty queue\n");
      exit(-1);
    }
  int ret = q->head->value;
  AdjacencyList* new_head = q->head->next;
  free(q->head);
  q->head = new_head;
  if (new_head == NULL)
    {
      q->tail = NULL;
    }
  return ret;
}

bool is_queue_empty(Queue* q)
{
  return q->head == NULL;
}


void input_pair(int* a, int* b)
{
  scanf("%d %d", a, b);
}

void connect(SDSGraph* g, int a, int b)
{
  a -= 1;
  b -= 1;
  AdjacencyList* al = malloc(sizeof(AdjacencyList));
  al->value = b;
  al->next = g->adjacency_lists[a];
  g->adjacency_lists[a] = al;

  al = malloc(sizeof(AdjacencyList));
  al->value = a;
  al->next = g->adjacency_lists[b];
  g->adjacency_lists[b] = al;
}

SDSGraph* init_sdsgraph(int size)
{
  SDSGraph* g = malloc(sizeof(SDSGraph));
  g->size = size;
  g->adjacency_lists = malloc(sizeof(AdjacencyList*) * size);
  for (int i=0; i<size; i++)
    {
      g->adjacency_lists[i] = NULL;
    }
  return g;
}

int traverse_bfs_to_depth(SDSGraph* g, int start_idx, int depth)
{
  int curr, last=start_idx, tail, level=0, count=1;
  bool a[g->size];
  for(int i=0; i<g->size; i++)
    {
      a[i] = false;
    }
  a[start_idx] = true;
  Queue* q = init_queue();
  enqueue(q, start_idx);
  while(!is_queue_empty(q))
    {
      curr = dequeue(q);
      AdjacencyList* al = g->adjacency_lists[curr];
      while(al != NULL)
        {
          if(!a[al->value])
            {
              tail = al->value;
              a[tail] = true;
              enqueue(q, tail);
              count++;
           }
          al = al->next;
        }
      if (curr == last) {
        level++;
        last = tail;
      }
      if (level == depth) {
        break;
      }
    }
  return count;
}


int main()
{
  int vertices, edges;
  input_pair(&vertices, &edges);
  SDSGraph* g = init_sdsgraph(vertices);
  for(int i=0;i<edges;i++)
    {
      int a, b;
      input_pair(&a, &b);
      connect(g, a, b);
    }
  for(int i=0; i<vertices; i++)
    {
      printf("%d: %0.2f%%\n", i+1, traverse_bfs_to_depth(g, i, 6) * 100.0 /vertices);
    }
}
