#pragma once
#include <vector>
#include "uclid/Body.h"

class PhysicsWorld {
private:
	std::vector<Body> bodies;
	Vector3 gravity = Vector3(0.0, -9.81, 0.0);

public:
	void addBody(const Body& body);

	void step(double dt);

	const std::vector<Body>& getBodies() const;
};
