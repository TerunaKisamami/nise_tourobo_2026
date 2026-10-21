# generated from rosidl_generator_py/resource/_idl.py.em
# with input from tourobo_2026_interfaces:action/BallArmOperation.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_BallArmOperation_Goal(type):
    """Metaclass of message 'BallArmOperation_Goal'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_Goal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__goal
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__goal
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__goal
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__goal
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__goal

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_Goal(metaclass=Metaclass_BallArmOperation_Goal):
    """Message class 'BallArmOperation_Goal'."""

    __slots__ = [
        '_target_arm',
        '_is_open',
        '_current_state',
        '_carry',
        '_push_state',
        '_shoot_angle_state',
        '_is_left_arm_open',
        '_is_right_arm_open',
    ]

    _fields_and_field_types = {
        'target_arm': 'int32',
        'is_open': 'boolean',
        'current_state': 'int32',
        'carry': 'int32',
        'push_state': 'int32',
        'shoot_angle_state': 'int32',
        'is_left_arm_open': 'boolean',
        'is_right_arm_open': 'boolean',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.target_arm = kwargs.get('target_arm', int())
        self.is_open = kwargs.get('is_open', bool())
        self.current_state = kwargs.get('current_state', int())
        self.carry = kwargs.get('carry', int())
        self.push_state = kwargs.get('push_state', int())
        self.shoot_angle_state = kwargs.get('shoot_angle_state', int())
        self.is_left_arm_open = kwargs.get('is_left_arm_open', bool())
        self.is_right_arm_open = kwargs.get('is_right_arm_open', bool())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.target_arm != other.target_arm:
            return False
        if self.is_open != other.is_open:
            return False
        if self.current_state != other.current_state:
            return False
        if self.carry != other.carry:
            return False
        if self.push_state != other.push_state:
            return False
        if self.shoot_angle_state != other.shoot_angle_state:
            return False
        if self.is_left_arm_open != other.is_left_arm_open:
            return False
        if self.is_right_arm_open != other.is_right_arm_open:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def target_arm(self):
        """Message field 'target_arm'."""
        return self._target_arm

    @target_arm.setter
    def target_arm(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'target_arm' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'target_arm' field must be an integer in [-2147483648, 2147483647]"
        self._target_arm = value

    @builtins.property
    def is_open(self):
        """Message field 'is_open'."""
        return self._is_open

    @is_open.setter
    def is_open(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'is_open' field must be of type 'bool'"
        self._is_open = value

    @builtins.property
    def current_state(self):
        """Message field 'current_state'."""
        return self._current_state

    @current_state.setter
    def current_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'current_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'current_state' field must be an integer in [-2147483648, 2147483647]"
        self._current_state = value

    @builtins.property
    def carry(self):
        """Message field 'carry'."""
        return self._carry

    @carry.setter
    def carry(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'carry' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'carry' field must be an integer in [-2147483648, 2147483647]"
        self._carry = value

    @builtins.property
    def push_state(self):
        """Message field 'push_state'."""
        return self._push_state

    @push_state.setter
    def push_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'push_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'push_state' field must be an integer in [-2147483648, 2147483647]"
        self._push_state = value

    @builtins.property
    def shoot_angle_state(self):
        """Message field 'shoot_angle_state'."""
        return self._shoot_angle_state

    @shoot_angle_state.setter
    def shoot_angle_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'shoot_angle_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'shoot_angle_state' field must be an integer in [-2147483648, 2147483647]"
        self._shoot_angle_state = value

    @builtins.property
    def is_left_arm_open(self):
        """Message field 'is_left_arm_open'."""
        return self._is_left_arm_open

    @is_left_arm_open.setter
    def is_left_arm_open(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'is_left_arm_open' field must be of type 'bool'"
        self._is_left_arm_open = value

    @builtins.property
    def is_right_arm_open(self):
        """Message field 'is_right_arm_open'."""
        return self._is_right_arm_open

    @is_right_arm_open.setter
    def is_right_arm_open(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'is_right_arm_open' field must be of type 'bool'"
        self._is_right_arm_open = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_BallArmOperation_Result(type):
    """Metaclass of message 'BallArmOperation_Result'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_Result')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__result
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__result
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__result
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__result
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__result

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_Result(metaclass=Metaclass_BallArmOperation_Result):
    """Message class 'BallArmOperation_Result'."""

    __slots__ = [
        '_success',
        '_next_state',
        '_next_carry',
        '_next_push_state',
        '_next_shoot_angle_state',
        '_next_is_left_arm_open',
        '_next_is_right_arm_open',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'next_state': 'int32',
        'next_carry': 'int32',
        'next_push_state': 'int32',
        'next_shoot_angle_state': 'int32',
        'next_is_left_arm_open': 'boolean',
        'next_is_right_arm_open': 'boolean',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.next_state = kwargs.get('next_state', int())
        self.next_carry = kwargs.get('next_carry', int())
        self.next_push_state = kwargs.get('next_push_state', int())
        self.next_shoot_angle_state = kwargs.get('next_shoot_angle_state', int())
        self.next_is_left_arm_open = kwargs.get('next_is_left_arm_open', bool())
        self.next_is_right_arm_open = kwargs.get('next_is_right_arm_open', bool())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.success != other.success:
            return False
        if self.next_state != other.next_state:
            return False
        if self.next_carry != other.next_carry:
            return False
        if self.next_push_state != other.next_push_state:
            return False
        if self.next_shoot_angle_state != other.next_shoot_angle_state:
            return False
        if self.next_is_left_arm_open != other.next_is_left_arm_open:
            return False
        if self.next_is_right_arm_open != other.next_is_right_arm_open:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def success(self):
        """Message field 'success'."""
        return self._success

    @success.setter
    def success(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'success' field must be of type 'bool'"
        self._success = value

    @builtins.property
    def next_state(self):
        """Message field 'next_state'."""
        return self._next_state

    @next_state.setter
    def next_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'next_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'next_state' field must be an integer in [-2147483648, 2147483647]"
        self._next_state = value

    @builtins.property
    def next_carry(self):
        """Message field 'next_carry'."""
        return self._next_carry

    @next_carry.setter
    def next_carry(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'next_carry' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'next_carry' field must be an integer in [-2147483648, 2147483647]"
        self._next_carry = value

    @builtins.property
    def next_push_state(self):
        """Message field 'next_push_state'."""
        return self._next_push_state

    @next_push_state.setter
    def next_push_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'next_push_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'next_push_state' field must be an integer in [-2147483648, 2147483647]"
        self._next_push_state = value

    @builtins.property
    def next_shoot_angle_state(self):
        """Message field 'next_shoot_angle_state'."""
        return self._next_shoot_angle_state

    @next_shoot_angle_state.setter
    def next_shoot_angle_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'next_shoot_angle_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'next_shoot_angle_state' field must be an integer in [-2147483648, 2147483647]"
        self._next_shoot_angle_state = value

    @builtins.property
    def next_is_left_arm_open(self):
        """Message field 'next_is_left_arm_open'."""
        return self._next_is_left_arm_open

    @next_is_left_arm_open.setter
    def next_is_left_arm_open(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'next_is_left_arm_open' field must be of type 'bool'"
        self._next_is_left_arm_open = value

    @builtins.property
    def next_is_right_arm_open(self):
        """Message field 'next_is_right_arm_open'."""
        return self._next_is_right_arm_open

    @next_is_right_arm_open.setter
    def next_is_right_arm_open(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'next_is_right_arm_open' field must be of type 'bool'"
        self._next_is_right_arm_open = value


# Import statements for member types

# already imported above
# import rosidl_parser.definition


class Metaclass_BallArmOperation_Feedback(type):
    """Metaclass of message 'BallArmOperation_Feedback'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_Feedback')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__feedback
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__feedback
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__feedback
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__feedback
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__feedback

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_Feedback(metaclass=Metaclass_BallArmOperation_Feedback):
    """Message class 'BallArmOperation_Feedback'."""

    __slots__ = [
    ]

    _fields_and_field_types = {
    }

    SLOT_TYPES = (
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_BallArmOperation_SendGoal_Request(type):
    """Metaclass of message 'BallArmOperation_SendGoal_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_SendGoal_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__send_goal__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__send_goal__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__send_goal__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__send_goal__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__send_goal__request

            from tourobo_2026_interfaces.action import BallArmOperation
            if BallArmOperation.Goal.__class__._TYPE_SUPPORT is None:
                BallArmOperation.Goal.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_SendGoal_Request(metaclass=Metaclass_BallArmOperation_SendGoal_Request):
    """Message class 'BallArmOperation_SendGoal_Request'."""

    __slots__ = [
        '_goal_id',
        '_goal',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'goal': 'tourobo_2026_interfaces/BallArmOperation_Goal',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['tourobo_2026_interfaces', 'action'], 'BallArmOperation_Goal'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Goal
        self.goal = kwargs.get('goal', BallArmOperation_Goal())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.goal_id != other.goal_id:
            return False
        if self.goal != other.goal:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def goal(self):
        """Message field 'goal'."""
        return self._goal

    @goal.setter
    def goal(self, value):
        if __debug__:
            from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Goal
            assert \
                isinstance(value, BallArmOperation_Goal), \
                "The 'goal' field must be a sub message of type 'BallArmOperation_Goal'"
        self._goal = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_BallArmOperation_SendGoal_Response(type):
    """Metaclass of message 'BallArmOperation_SendGoal_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_SendGoal_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__send_goal__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__send_goal__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__send_goal__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__send_goal__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__send_goal__response

            from builtin_interfaces.msg import Time
            if Time.__class__._TYPE_SUPPORT is None:
                Time.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_SendGoal_Response(metaclass=Metaclass_BallArmOperation_SendGoal_Response):
    """Message class 'BallArmOperation_SendGoal_Response'."""

    __slots__ = [
        '_accepted',
        '_stamp',
    ]

    _fields_and_field_types = {
        'accepted': 'boolean',
        'stamp': 'builtin_interfaces/Time',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Time'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.accepted = kwargs.get('accepted', bool())
        from builtin_interfaces.msg import Time
        self.stamp = kwargs.get('stamp', Time())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.accepted != other.accepted:
            return False
        if self.stamp != other.stamp:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def accepted(self):
        """Message field 'accepted'."""
        return self._accepted

    @accepted.setter
    def accepted(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'accepted' field must be of type 'bool'"
        self._accepted = value

    @builtins.property
    def stamp(self):
        """Message field 'stamp'."""
        return self._stamp

    @stamp.setter
    def stamp(self, value):
        if __debug__:
            from builtin_interfaces.msg import Time
            assert \
                isinstance(value, Time), \
                "The 'stamp' field must be a sub message of type 'Time'"
        self._stamp = value


class Metaclass_BallArmOperation_SendGoal(type):
    """Metaclass of service 'BallArmOperation_SendGoal'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_SendGoal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__ball_arm_operation__send_goal

            from tourobo_2026_interfaces.action import _ball_arm_operation
            if _ball_arm_operation.Metaclass_BallArmOperation_SendGoal_Request._TYPE_SUPPORT is None:
                _ball_arm_operation.Metaclass_BallArmOperation_SendGoal_Request.__import_type_support__()
            if _ball_arm_operation.Metaclass_BallArmOperation_SendGoal_Response._TYPE_SUPPORT is None:
                _ball_arm_operation.Metaclass_BallArmOperation_SendGoal_Response.__import_type_support__()


class BallArmOperation_SendGoal(metaclass=Metaclass_BallArmOperation_SendGoal):
    from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_SendGoal_Request as Request
    from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_SendGoal_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_BallArmOperation_GetResult_Request(type):
    """Metaclass of message 'BallArmOperation_GetResult_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_GetResult_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__get_result__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__get_result__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__get_result__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__get_result__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__get_result__request

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_GetResult_Request(metaclass=Metaclass_BallArmOperation_GetResult_Request):
    """Message class 'BallArmOperation_GetResult_Request'."""

    __slots__ = [
        '_goal_id',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.goal_id != other.goal_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_BallArmOperation_GetResult_Response(type):
    """Metaclass of message 'BallArmOperation_GetResult_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_GetResult_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__get_result__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__get_result__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__get_result__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__get_result__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__get_result__response

            from tourobo_2026_interfaces.action import BallArmOperation
            if BallArmOperation.Result.__class__._TYPE_SUPPORT is None:
                BallArmOperation.Result.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_GetResult_Response(metaclass=Metaclass_BallArmOperation_GetResult_Response):
    """Message class 'BallArmOperation_GetResult_Response'."""

    __slots__ = [
        '_status',
        '_result',
    ]

    _fields_and_field_types = {
        'status': 'int8',
        'result': 'tourobo_2026_interfaces/BallArmOperation_Result',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['tourobo_2026_interfaces', 'action'], 'BallArmOperation_Result'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.status = kwargs.get('status', int())
        from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Result
        self.result = kwargs.get('result', BallArmOperation_Result())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.status != other.status:
            return False
        if self.result != other.result:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def status(self):
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'status' field must be of type 'int'"
            assert value >= -128 and value < 128, \
                "The 'status' field must be an integer in [-128, 127]"
        self._status = value

    @builtins.property
    def result(self):
        """Message field 'result'."""
        return self._result

    @result.setter
    def result(self, value):
        if __debug__:
            from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Result
            assert \
                isinstance(value, BallArmOperation_Result), \
                "The 'result' field must be a sub message of type 'BallArmOperation_Result'"
        self._result = value


class Metaclass_BallArmOperation_GetResult(type):
    """Metaclass of service 'BallArmOperation_GetResult'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_GetResult')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__ball_arm_operation__get_result

            from tourobo_2026_interfaces.action import _ball_arm_operation
            if _ball_arm_operation.Metaclass_BallArmOperation_GetResult_Request._TYPE_SUPPORT is None:
                _ball_arm_operation.Metaclass_BallArmOperation_GetResult_Request.__import_type_support__()
            if _ball_arm_operation.Metaclass_BallArmOperation_GetResult_Response._TYPE_SUPPORT is None:
                _ball_arm_operation.Metaclass_BallArmOperation_GetResult_Response.__import_type_support__()


class BallArmOperation_GetResult(metaclass=Metaclass_BallArmOperation_GetResult):
    from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_GetResult_Request as Request
    from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_GetResult_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_BallArmOperation_FeedbackMessage(type):
    """Metaclass of message 'BallArmOperation_FeedbackMessage'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation_FeedbackMessage')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ball_arm_operation__feedback_message
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ball_arm_operation__feedback_message
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ball_arm_operation__feedback_message
            cls._TYPE_SUPPORT = module.type_support_msg__action__ball_arm_operation__feedback_message
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ball_arm_operation__feedback_message

            from tourobo_2026_interfaces.action import BallArmOperation
            if BallArmOperation.Feedback.__class__._TYPE_SUPPORT is None:
                BallArmOperation.Feedback.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class BallArmOperation_FeedbackMessage(metaclass=Metaclass_BallArmOperation_FeedbackMessage):
    """Message class 'BallArmOperation_FeedbackMessage'."""

    __slots__ = [
        '_goal_id',
        '_feedback',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'feedback': 'tourobo_2026_interfaces/BallArmOperation_Feedback',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['tourobo_2026_interfaces', 'action'], 'BallArmOperation_Feedback'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Feedback
        self.feedback = kwargs.get('feedback', BallArmOperation_Feedback())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.goal_id != other.goal_id:
            return False
        if self.feedback != other.feedback:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def feedback(self):
        """Message field 'feedback'."""
        return self._feedback

    @feedback.setter
    def feedback(self, value):
        if __debug__:
            from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Feedback
            assert \
                isinstance(value, BallArmOperation_Feedback), \
                "The 'feedback' field must be a sub message of type 'BallArmOperation_Feedback'"
        self._feedback = value


class Metaclass_BallArmOperation(type):
    """Metaclass of action 'BallArmOperation'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.BallArmOperation')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_action__action__ball_arm_operation

            from action_msgs.msg import _goal_status_array
            if _goal_status_array.Metaclass_GoalStatusArray._TYPE_SUPPORT is None:
                _goal_status_array.Metaclass_GoalStatusArray.__import_type_support__()
            from action_msgs.srv import _cancel_goal
            if _cancel_goal.Metaclass_CancelGoal._TYPE_SUPPORT is None:
                _cancel_goal.Metaclass_CancelGoal.__import_type_support__()

            from tourobo_2026_interfaces.action import _ball_arm_operation
            if _ball_arm_operation.Metaclass_BallArmOperation_SendGoal._TYPE_SUPPORT is None:
                _ball_arm_operation.Metaclass_BallArmOperation_SendGoal.__import_type_support__()
            if _ball_arm_operation.Metaclass_BallArmOperation_GetResult._TYPE_SUPPORT is None:
                _ball_arm_operation.Metaclass_BallArmOperation_GetResult.__import_type_support__()
            if _ball_arm_operation.Metaclass_BallArmOperation_FeedbackMessage._TYPE_SUPPORT is None:
                _ball_arm_operation.Metaclass_BallArmOperation_FeedbackMessage.__import_type_support__()


class BallArmOperation(metaclass=Metaclass_BallArmOperation):

    # The goal message defined in the action definition.
    from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Goal as Goal
    # The result message defined in the action definition.
    from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Result as Result
    # The feedback message defined in the action definition.
    from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_Feedback as Feedback

    class Impl:

        # The send_goal service using a wrapped version of the goal message as a request.
        from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_SendGoal as SendGoalService
        # The get_result service using a wrapped version of the result message as a response.
        from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_GetResult as GetResultService
        # The feedback message with generic fields which wraps the feedback message.
        from tourobo_2026_interfaces.action._ball_arm_operation import BallArmOperation_FeedbackMessage as FeedbackMessage

        # The generic service to cancel a goal.
        from action_msgs.srv._cancel_goal import CancelGoal as CancelGoalService
        # The generic message for get the status of a goal.
        from action_msgs.msg._goal_status_array import GoalStatusArray as GoalStatusMessage

    def __init__(self):
        raise NotImplementedError('Action classes can not be instantiated')
