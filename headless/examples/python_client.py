#!/usr/bin/python3
# This example will see how changing one traffic signal affects trip times.
# Before running this script, start the API server:
#
# > cargo run --release --bin headless -- --port=1234

import json
# You may need to install https://requests.readthedocs.io
import requests


api = 'http://localhost:1234'
hours_to_sim = '12:00:00'


def main():
    # Make sure to start the simulation from the beginning
    print(
        'Did you just start the simulation? Time is currently',
        requests.get(f'{api}/sim/get-time').text,
    )

    print('Reset the simulation:', requests.get(f'{api}/sim/reset').text)
    print()

    # Run a few hours to get a baseline
    print('Simulating before any edits')
    trips1, delays1, thruput1 = run_experiment()
    print(
        f'Baseline: {len(trips1)} finished trips, total of {sum(trips1.values())} seconds'
    )

    print()

    # Find the average position of all active pedestrians
    agents = [
        x['pos']
        for x in requests.get(f'{api}/data/get-agent-positions').json()[
            'agents'
        ]
        if x['vehicle_type'] is None
    ]

    avg_lon = sum(x['longitude'] for x in agents) / len(agents)
    avg_lat = sum(x['latitude'] for x in agents) / len(agents)
    print(f'Average position of all active pedestrians: {avg_lon}, {avg_lat}')
    print()

    # Modify one traffic signal, doubling the duration of its second stage
    print('Modify a traffic signal')
    ts = requests.get(f'{api}/traffic-signals/get', params={'id': 67}).json()
    ts['stages'][1]['stage_type']['Fixed'] *= 2
    # Reset the simulation before applying the edit, since reset also clears edits.
    print('Reset the simulation:', requests.get(f'{api}/sim/reset').text)
    print(
        'Update a traffic signal:',
        requests.post(f'{api}/traffic-signals/set', json=ts).text,
    )

    print()

    # Repeat the experiment
    print('Simulating after the edits')
    trips2, delays2, thruput2 = run_experiment()
    print(
        f'Experiment: {len(trips2)} finished trips, total of {sum(trips2.values())} seconds'
    )

    print()

    # Compare -- did this help or not?
    print(
        f'{len(trips2) - len(trips1)} more trips finished after the edits (higher is better)'
    )

    print(
        f'Experiment was {sum(trips1.values()) - sum(trips2.values())} seconds faster, over all trips'
    )

    print()

    # Now we'll print some before/after stats per direction of travel through
    # the intersection
    col = '{:<40} {:>20} {:>20} {:>17} {:>17}'
    print(col.format('Direction', 'avg delay before',
                     'avg delay after', 'thruput before', 'thruput after'))
    for k in delays1.keys():
        print(col.format(k, delays1[k], delays2[k], thruput1[k], thruput2[k]))


# Returns (trips, delay, throughput)
def run_experiment():
    print(requests.get(f'{api}/sim/goto-time', params={'t': hours_to_sim}).text)
    raw_trips = requests.get(f'{api}/data/get-finished-trips').json()
    raw_delays = requests.get(
        f'{api}/traffic-signals/get-delays',
        params={'id': 67, 't1': '00:00:00', 't2': hours_to_sim},
    ).json()

    raw_thruput = requests.get(
        f'{api}/traffic-signals/get-cumulative-thruput', params={'id': 67}
    ).json()


    # Map trip ID to the duration (in seconds) of the trip. Filter out
    # cancelled trips.
    trips = {
        trip['id']: trip['duration']
        for trip in raw_trips
        if trip['duration'] is not None
    }

    # The direction is a dict, but Python can't handle dicts as keys. Stringify
    # the keys, also filtering out crosswalks and empty directions.
    delays = {}
    for k, v in raw_delays['per_direction']:
        k = stringify_direction(k)
        if k and v:
            delays[k] = '{:.1f}'.format(sum(v) / len(v))

    thruput = {}
    for k, v in raw_thruput['per_direction']:
        if k := stringify_direction(k):
            thruput[k] = v

    return (trips, delays, thruput)


def stringify_direction(direxn):
    if direxn['crosswalk']:
        return None
    return f"{stringify_road(direxn['from'])} -> {stringify_road(direxn['to'])}"


def stringify_road(directed_road):
    return f"Road #{directed_road['id']} ({directed_road['dir']})"


if __name__ == '__main__':
    main()
