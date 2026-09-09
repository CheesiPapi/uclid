#pragma once
#include "Vector3.h"

struct Body {
	double mass;
	Vector3 position;
	Vector3 velocity;
	Vector3 forceAccumulator; // Stores all forces applied this frame
	
	Body(double m, Vector3 pos) : mass(m), position(pos), velocity(0,0,0), forceAccumulator(0,0,0) {}

	// Call this to apply gravity, drag, springs, etc.
	void applyForce(const Vector3& force) {
		forceAccumulator += force;
	}

	// Clears forces at the end of the time step
	void clearForce {
		forceAccumulator = Vector3(0, 0, 0);
	}
		
};
