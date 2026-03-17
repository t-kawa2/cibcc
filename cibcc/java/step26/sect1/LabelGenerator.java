public class LabelGenerator {

	private static int cnt = 0;

	public static String newLabel() {
		String buf = String.format(".L.data.%d", cnt++);
		return buf;
	}
}
