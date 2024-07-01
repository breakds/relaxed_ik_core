# This is should produce exactly the same result as ``relaxed_ik_bin``
import numpy as np
from relaxed_ik_lib import RelaxedIK


if __name__ == "__main__":
    path = "/home/breakds/syncthing/workspace/hobot/relaxed_ik/wx250s.yaml"
    ik = RelaxedIK(path)
    p, q = ik.current_goal
    np.set_printoptions(precision=5, suppress=True)
    print("Initial Pose:", p, q)
    for _ in range(10):
        p += np.array([0., 0.01, 0.])
        joint_pos = ik.solve(p, q)
        print(joint_pos)
