"""Functions used in preparing Guido's gorgeous lasagna.

Learn about Guido, the creator of the Python language: https://en.wikipedia.org/wiki/Guido_van_Rossum
"""

EXPECTED_BAKE_TIME = 40
PREPARATION_TIME = 2

def bake_time_remaining(elapsed_bake_time: int) -> int:
    """Calculate the bake time remaining.

    :param elapsed_bake_time: int - baking time already elapsed.
    :return: int - remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.
    """

    if (rem := EXPECTED_BAKE_TIME - elapsed_bake_time) >= 0:
        return rem
    return None



def preparation_time_in_minutes(number_of_layers: int) -> int:
    """Calculate the preparation time.

    :param number_of_layers: int - # of layers
    :return: int - total preparation time
    """
    return number_of_layers * PREPARATION_TIME

def elapsed_time_in_minutes(number_of_layers: int, elapsed_bake_time):
    """Calculate total elapsed time.

    :param number_of_layers: int - # of layers
    :param elapsed_bake_time: int - elapsed bake time
    :return: int - total elapsed time
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
