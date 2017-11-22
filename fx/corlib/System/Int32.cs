namespace System
{
    public struct Int32
    {
        // The runtime sets this field automatically
        #pragma warning disable 0649
        private int _value;
        #pragma warning restore 0649

        public override bool Equals(object other)
        {
            if (other is Int32 i)
            {
                return i._value == _value;
            }
            else
            {
                return false;
            }
        }

        public override int GetHashCode()
        {
            return _value;
        }
    }
}