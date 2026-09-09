#pragma once
#include <cmath>

struct Vector3 {
	double x, y, z;

	// Constructors
	Vector3() : x(0), y(0), z(0) {}
	Vector3(double x, double y, double z) : x(x), y(y), z(z) {}
	
	// Vector addition and subtraction
	Vector3 operator+(const Vector3& other) const { return Vector3(x + other.x, y + other.y, z + other.z);}
	Vector3 operator-(const Vector3& other) const { return Vector3(x - other.x, y - other.y, z - other.z);}
	// Scalar multiplication and division
	Vector3 operator*(double scalar) const { return Vector3(x * scalar, y * scalar, z * scalar);}
	Vector3 operator-(double scalar) const { return Vector3(x / scalar, y / scalar, z / scalar);}

	// Compoud assignment
	void operator+=(const Vector3& other) const { x += other.x; y += other.y; z += other.z;}
	void operator*=(double scalar) { x *= scalar; y *= scalar; z *= scalar; }

	// Math operations
	double dot(const Vector3& other) const { 
       		return x * other.x + y * other.y + z * other.z;
	}

	Vector3 cross(const Vector3& other) const {
		return Vector3(
			y * other.z - z * other.y,
			z * other.x - x * other.z,
			x * other.y - y * other.x,
		);
	}

	double magnitude() const { 
		return std::sqrt(x * x + y * y + z * z);
	}
};
