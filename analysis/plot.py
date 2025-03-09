import json
import numpy as np
import matplotlib.pyplot as plt


def parse(kernel, calc_cnt=1):
    with open(f"{kernel}.json", "r") as file:
        data = json.load(file)

    t = []
    p_diff_max = []
    v_diff_max = []
    for i in data:
        t.append(i["dt"] * calc_cnt)
        p_diff_max.append(i["p_diff_max"])
        v_diff_max.append(i["v_diff_max"])

    t = np.array(t)
    p_diff_max = np.array(p_diff_max)
    v_diff_max = np.array(v_diff_max)

    start_idx = 18

    # Fit a line to the log-log data
    log_t = np.log(t[start_idx:])
    log_p_diff_max = np.log(p_diff_max[start_idx:])
    log_v_diff_max = np.log(v_diff_max[start_idx:])

    # Fit the data using a linear regression (polyfit with degree 1)
    p_fit = np.polyfit(log_t, log_p_diff_max, 1)
    v_fit = np.polyfit(log_t, log_v_diff_max, 1)

    # Create the fit lines
    p_fit_line = np.exp(p_fit[1]) * np.array(t) ** p_fit[0]
    v_fit_line = np.exp(v_fit[1]) * np.array(t) ** v_fit[0]

    # Plot the fit lines
    color = plt.plot(
        t,
        p_fit_line,
        linestyle="--",
        label=f"{kernel} Fit: p_diff (slope={p_fit[0]:.2f})",
    )[0].get_color()
    plt.plot(t, p_diff_max, label=f"{kernel} p_diff", color=color)
    plt.scatter(t, p_diff_max, color=color)

    # color = plt.plot(
    #     t,
    #     v_fit_line,
    #     linestyle="--",
    #     label=f"{kernel} Fit: v_diff (slope={v_fit[0]:.2f})",
    # )[0].get_color()
    # plt.plot(t, v_diff_max, label=f"{kernel} v_diff", color=color)
    # plt.scatter(t, v_diff_max, color=color)

    plt.xlabel("dt")

    plt.xscale("log")
    plt.yscale("log")


if __name__ == "__main__":
    parse("yoshida4_relative", 3)
    parse("yoshida4", 3)
    parse("vel_verlet_relative")
    parse("vel_verlet")
    parse("symplectic_euler_relative")
    parse("symplectic_euler")
    parse("rk4", 4)
    plt.ylim(bottom=1e-10)
    box = plt.gca().get_position()
    plt.gca().set_position([box.x0, box.y0, box.width * 0.8, box.height])
    plt.legend(loc="center left", bbox_to_anchor=(1, 0.5))
    plt.show()
