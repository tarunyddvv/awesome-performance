#include <stdio.h>
#include <stdlib.h>
#include <iostream>

using namespace std;

class Rectangle {
    private:
        int length;
        int breadth;

    public:
        // INFO: constructors; without and with arguments
        Rectangle() {
            length = 0;
            breadth = 0;
        }

        Rectangle(int l, int b) {
            length = l;
            breadth = b;
        }

        // INFO: facilitators
        int peri() {
            return 2 * (length + breadth);
        }

        int area() {
            return length * breadth;
        }

        // INFO: getters and setters
        int getLength() {
            return length;
        }

        int getBreadth() {
            return breadth;
        }

        void setLength(int len) {
            length = len;
        }

        void setBreadth(int bth) {
            breadth = bth;
        }

        // INFO: destructors
        ~Rectangle() {
            cout<<"Destructor";
        };
};

int main() {
    Rectangle r(10, 20);

    cout<<"area: "<<r.area()<<endl;
    cout<<"perimeter: "<<r.peri()<<endl;
    cout<<"length: "<<r.getLength()<<endl;
    cout<<"breadth: "<<r.getBreadth()<<endl;

    r.setBreadth(50);
    r.setLength(80);

    cout<<"area: "<<r.area()<<endl;
    cout<<"perimeter: "<<r.peri()<<endl;
    cout<<"length: "<<r.getLength()<<endl;
    cout<<"breadth: "<<r.getBreadth()<<endl;

    return 0;
}
