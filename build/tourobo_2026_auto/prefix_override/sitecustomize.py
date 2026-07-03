import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/hatsu/Robobobobobobo/tourobo_2026/install/tourobo_2026_auto'
