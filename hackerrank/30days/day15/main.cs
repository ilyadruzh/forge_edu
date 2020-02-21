using System;
class Node
{
    public int data;
    public Node next;
    public Node(int d)
    {
        data = d;
        next = null;
    }

}
class Solution
{

    public static Node insert(Node head, int data)
    {
        Node x = new Node(data);

        if (head == null)
        {
            return x;
        }
        else
        {
            Node start = head;
            do
            {
                if (start.next == null)
                {
                    start.next = x;
                    break;
                }
                start = start.next;
            } while (start != null);

            return head;
        }
    }

    public static void display(Node head)
    {
        Node start = head;
        while (start != null)
        {
            Console.Write(start.data + " ");
            start = start.next;
        }
    }
    static void Main(String[] args)
    {

        Node head = null; // Node(2)
        int T = Int32.Parse(Console.ReadLine());
        while (T-- > 0)
        {
            int data = Int32.Parse(Console.ReadLine());
            head = insert(head, data);
        }
        display(head);
    }
}
