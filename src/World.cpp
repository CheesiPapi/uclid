#include "uclid/World.h"

void PhysicsWorld::addBody(const Body& body) {
	bodies.push_back(body);
}

void PhysicsWorld::step(double dt) {
	// 1. Apply global forces
	for (auto& body : bodies) {
		body.applyForce(gravity * body.mass);
	}

	// 2. Integrate
	for (auto& body : bodies) {
		if (body.mass <= 0.0) continue; // Skip static objects

		Vector3 acceleration = body.forceAccumulator / body.mass;

		// Semi-Implicit Euler: update velocity first, then position
		body.velocity += acceleration * dt;
		body.position += body.velocity * dt;
	}
}

const std::vector<Body>& PhysicsWorld::getBodies() const {
	return bodies;
}
