List _Merge(List L1, List L2)
{
  if (L1 == NULL)
    {
      return L2;
    }
  if (L2 == NULL)
    {
      return L1;
    }
  if (L1->Data < L2->Data)
    {
      L1->Next = _Merge(L1->Next, L2);
      return L1;
    }
  else
    {
      L2->Next = _Merge(L1, L2->Next);
      return L2;
    }
}

List Merge(List L1, List L2)
{
  PtrToNode head = malloc(sizeof(struct Node));
  head->Next = _Merge(L1->Next, L2->Next);
  L1->Next = NULL;
  L2->Next = NULL;
  return head;
}
