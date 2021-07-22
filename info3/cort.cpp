#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

static bool sort_function (double p, double q);

int main()
{
    fstream myfile;

    // opening file
    myfile.open("data.txt");
    if (myfile.fail())
    {
        cout << "Error Opening File" << endl;
        exit(1);
    }

    // reading data and storing data
    vector<double> x;
    double tmp;
    while (myfile >> tmp)
    {
        x.push_back(tmp);
    }
    myfile.close();
    cout << x.size() << " number of values in file" << endl;

    // unsorted print data from vector
    for (vector<double>::iterator it = x.begin(); it != x.end(); ++i)
    {
        cout << *it << endl;
    }

    // sorted print data from vector
    sort(x.begin(), x.end(), sort_function);
    for (vector<double>::iterator it = x.begin(); it != x.end(); ++i)
    {
        cout << *it << endl;
    }

    return 0;
}

static bool sort_function (double p, double q)
{
    return p > q;
}