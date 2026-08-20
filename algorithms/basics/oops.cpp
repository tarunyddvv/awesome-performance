#include <stdio.h>
#include <stdlib.h>
#include <iostream>

using namespace std;

template<class T>
class Rectangle {
    private:
        T length;
        T breadth;

    public:
        // INFO: constructors; without and with arguments
        Rectangle() {
            length = 0;
            breadth = 0;
        }

        Rectangle(T l, T b) {
            length = l;
            breadth = b;
        }

        // INFO: facilitators
        T peri() {
            return 2 * (length + breadth);
        }

        T area() {
            return length * breadth;
        }

        // INFO: getters and setters
        T getLength() {
            return length;
        }

        T getBreadth() {
            return breadth;
        }

        void setLength(T len) {
            length = len;
        }

        void setBreadth(T bth) {
            breadth = bth;
        }

        // INFO: destructors
        ~Rectangle() {
            cout<<"Destructor"<<endl;
        };
};

int main() {
    Rectangle<int> r(10, 20);
    Rectangle<float> r2(50.0, 20.0);

    cout<<"area: "<<r.area()<<endl;
    cout<<"perimeter: "<<r.peri()<<endl;
    cout<<"length: "<<r.getLength()<<endl;
    cout<<"breadth: "<<r.getBreadth()<<endl;

    r2.setBreadth(50.0);
    r2.setLength(80.0);

    cout<<"area: "<<r2.area()<<endl;
    cout<<"perimeter: "<<r2.peri()<<endl;
    cout<<"length: "<<r2.getLength()<<endl;
    cout<<"breadth: "<<r2.getBreadth()<<endl;

    return 0;
}
